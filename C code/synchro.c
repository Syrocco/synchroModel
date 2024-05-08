#include "synchro.h"
#include "mersenne.c"

#ifndef M_PI
#    define M_PI 3.14159265358979323846
#endif

double t = 0;
int N = 10;

double r = 0.5;

double amp = 0.29;
double w = 1;
double h = 3;

double g = -0.1;
double k = 1001;
double gamm = 1;

double res = 0.9;
double dt;

particle* particles;

double delta = 0.00000001;

int hard = 1;

int parallel = 0;

FILE *fichier;
char filename[256];

int main(int argc, char *argv[]){
    dt = 0.001*2*M_PI*w;
    int c;

	while ((c = getopt(argc, argv, "N:A:h:a:w:g:k:H:G:p")) != -1){
		switch(c){
			case 'N':
				sscanf(optarg, "%d", &N);
				break;
			case 'A':
				sscanf(optarg, "%lf", &amp);
				break;
			case 'h':
				sscanf(optarg, "%lf", &h);
				break;
			case 'a':
				sscanf(optarg, "%lf", &res);
				break;
			case 'w':
				sscanf(optarg, "%lf", &w);
				break;
			case 'g':
				sscanf(optarg, "%lf", &g);
				break;
			case 'k':
				sscanf(optarg, "%lf", &k);
				break;
			case 'H':
				sscanf(optarg, "%d", &hard);
				break;
            case 'G':
				sscanf(optarg, "%lf", &gamm);
				break;
            case 'p':
                parallel = 1;
          break;
		}
	}
    
    if (parallel) {
        omp_set_num_threads(6);
    } else {
        omp_set_num_threads(1);
    }
    

    init();
    sprintf(filename, "phi_0.2freq_53T_%lfh_%lf.dumpL", amp, h);
    fichier = fopen(filename, "w");
    integrate(1000000);
    for (int i = 0; i < 100000; i++){
        integrate(1000);
        saveTXT();
    }
    free(particles);
    fclose(fichier);
    return 0;
}

void init(){
    particles = calloc(N, sizeof(particle));
    for (int i = 0; i < N; i++){
        particle* p = particles + i;
        p->r = r;
        p->z = h/2;
        p->v = drand(-30, 30);
    }

}

void integrate(int timesteps){
    for (int j = 0; j < timesteps; j++){
        #pragma omp parallel for if(parallel)
        for (int i = 0; i < N; i++){

            particle* p = particles + i;
            double f = g;
            double dist = floorDist(p);

            if (dist < 0){
                if (hard){
                    p->z = p->z - 2*dist;
                    p->v = p->v + (1 + res)*(plateVel(t) - p->v);
                }
                else
                    f -= dist*k + p->v*gamm; 
            }
            else{
                dist = ceilDist(p);
                if (dist > 0){
                    if (hard){
                        p->z = p->z - 2*dist;
                        p->v = p->v + (1 + res)*(plateVel(t) - p->v);
                    }
                    else
                      f -= dist*k + p->v*gamm;
                }
            }
            if (hard){
                p->z += 0.5*g*dt*dt + p->v*dt;
                p->v += g*dt;
            }
            else{
                p->v += f*dt;
                p->z += p->v*dt; 
            }
        }   
        t += dt;
    }
}




//Helpers
double drand(double min, double max){
    return (genrand()*(max - min)) + min;
}

inline double floorPos(double time){
    return amp*sin(w*time);
}

inline double ceilPos(double time){
    return floorPos(time) + h;
}

inline double plateVel(double time){
    return amp*w*cos(w*time);
}

double ceilDist(particle* p){
    return p->z - (ceilPos(t) - p->r);

}

double floorDist(particle* p){
    return p->z - (floorPos(t) + p->r);
}


void saveTXT(){
    fprintf(fichier, "ITEM: TIMESTEP\n%lf\nITEM: NUMBER OF ATOMS\n%d\nITEM: BOX BOUNDS xy xz yz\n %lf %lf 0\n %lf %lf 0\n0 2 0\nITEM: ATOMS id x y vz radius\n", t, N, -2*r, (double)N, floorPos(t), ceilPos(t));
    for(int i = 0; i < N; i++){
        particle* p = particles + i;
        fprintf(fichier, "%d %.2lf %lf %lf %lf\n", i, (double)i, p->z , p->v, p->r);
    }
}