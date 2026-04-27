public class AppleSmoothie extends Drink {
    public AppleSmoothie() {
        super(1.50f, 431, false);
    }

    @Override
    public float getPrice() {
        return 1.50f;
    }

    @Override
    public int getCalories() {
        return 431;
    }
}