#include<math.h>
#include<stdlib.h>
#include<getopt.h>
#include<omp.h>

 typedef struct particle particle;
 struct particle{
    double z, v, r;
 };


void init();
double drand(double min, double max);
void integrate(int timesteps);
double floorPos(double time);
double ceilPos(double time);
double plateVel(double time);
double ceilDist(particle* p);
double floorDist(particle* p);
void saveTXT();