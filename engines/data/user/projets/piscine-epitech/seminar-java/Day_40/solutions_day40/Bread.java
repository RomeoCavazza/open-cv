public abstract class Bread implements Food {
    private float price;
    private int calories;
    private int bakingTime = 0;

    public Bread(float price, int calories) {
        this.price = price;
        this.calories = calories;
        this.bakingTime = 0;
    }

    public Bread(float price, int calories, int bakingTime) {
        this.price = price;
        this.calories = calories;
        this.bakingTime = bakingTime;
    }

    public float getPrice() {
        return price;
    }

    public int getCalories() {
        return calories;
    }

    public int getBakingTime() {
        return bakingTime;
    }
}