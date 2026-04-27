public class Panini extends Sandwich {
    public Panini() {
        super(true);
        getIngredients().add("tomato");
        getIngredients().add("salad");
        getIngredients().add("cucumber");
        getIngredients().add("avocado");
        getIngredients().add("cheese");
    }

    @Override
    public float getPrice() {
        return 3.50f;
    }

    @Override
    public int getCalories() {
        return 120;
    }
}
