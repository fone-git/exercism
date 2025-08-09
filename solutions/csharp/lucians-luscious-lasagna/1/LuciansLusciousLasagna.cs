class Lasagna
{
    public int ExpectedMinutesInOven()
    {
        return 40;
    }

    public int RemainingMinutesInOven(int actual)
    {
        return this.ExpectedMinutesInOven() - actual;
    }

    public int PreparationTimeInMinutes(int layerCount)
    {
        return layerCount * 2;
    }

    public int ElapsedTimeInMinutes(int layers, int ovenTime)
    {
        return this.PreparationTimeInMinutes(layers) + ovenTime;
    }
}
