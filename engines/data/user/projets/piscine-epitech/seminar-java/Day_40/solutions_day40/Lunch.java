public class Lunch<D extends Drink, M extends Sandwich> extends Menu<D, M> {
    public Lunch(D drink, M sandwich) {
        super(drink, sandwich);
    }
}
