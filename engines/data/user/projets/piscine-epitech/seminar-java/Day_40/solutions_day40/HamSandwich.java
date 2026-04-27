public class HamSandwich extends Sandwich {
    public HamSandwich() {
        super(false);
        getIngredients().add("tomato");
        getIngredients().add("salad");
        getIngredients().add("cheese");
        getIngredients().add("ham");
        getIngredients().add("butter");
    }

    @Override
    public float getPrice() {
        return 4.00f;
    }

    @Override
    public int getCalories() {
        return 230;
    }
}
