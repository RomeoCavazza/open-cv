public class Coke extends Drink {
    public Coke() {
        super(1.20f, 105, true);
    }

    @Override
    public float getPrice() {
        return 1.20f;
    }

    @Override
    public int getCalories() {
        return 105;
    }
}