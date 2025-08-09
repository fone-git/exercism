class Lasagna
{
    public int ExpectedMinutesInOven() => 40;

    public int RemainingMinutesInOven(int actual) => this.ExpectedMinutesInOven() - actual;

    public int PreparationTimeInMinutes(int layerCount) => layerCount * 2;

    public int ElapsedTimeInMinutes(int layers, int ovenTime) => this.PreparationTimeInMinutes(layers) + ovenTime;
}
