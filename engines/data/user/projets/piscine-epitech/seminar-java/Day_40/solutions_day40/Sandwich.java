import java.util.ArrayList;
import java.util.List;

public abstract class Sandwich implements Food {
    private boolean vegetarian;
    private List<String> ingredients;

    public Sandwich(boolean vegetarian) {
        this.vegetarian = vegetarian;
        this.ingredients = new ArrayList<>();
    }

    public boolean isVegetarian() {
        return vegetarian;
    }

    public List<String> getIngredients() {
        return ingredients;
    }

    public abstract float getPrice();
    public abstract int getCalories();
}