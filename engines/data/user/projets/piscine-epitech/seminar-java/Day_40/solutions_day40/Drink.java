public abstract class Drink implements Food {
    private boolean aCan;

    public Drink(float price, int calories, boolean aCan) {
        this.aCan = aCan;
    }

    public boolean isACan() {
        return aCan;
    }

    public abstract float getPrice();
    public abstract int getCalories();
}