public class Breakfast<D extends Drink, M extends Bread> extends Menu<D, M> {
    public Breakfast(D drink, M bread) {
        super(drink, bread);
    }
}
