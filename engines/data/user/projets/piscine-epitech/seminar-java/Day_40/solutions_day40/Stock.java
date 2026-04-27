import java.util.HashMap;
import java.util.Map;

public class Stock {
    private Map<Class<? extends Food>, Integer> stock;

    public Stock() {
        this.stock = new HashMap<>();
        // Initialiser tous les types d'aliments avec 100 items
        this.stock.put(FrenchBaguette.class, 100);
        this.stock.put(SoftBread.class, 100);
        this.stock.put(AppleSmoothie.class, 100);
        this.stock.put(Coke.class, 100);
        this.stock.put(HamSandwich.class, 100);
        this.stock.put(Panini.class, 100);
        this.stock.put(Cookie.class, 100);
        this.stock.put(CheeseCake.class, 100);
    }

    public int getNumberOf(Class<? extends Food> food) throws NoSuchFoodException {
        if (!stock.containsKey(food)) {
            throw new NoSuchFoodException("No such food type: " + food.getName() + ".");
        }
        return stock.get(food);
    }

    public boolean add(Class<? extends Food> food) throws NoSuchFoodException {
        if (!stock.containsKey(food)) {
            throw new NoSuchFoodException("No such food type: " + food.getName() + ".");
        }
        stock.put(food, stock.get(food) + 1);
        return true;
    }

    public boolean remove(Class<? extends Food> food) throws NoSuchFoodException {
        if (!stock.containsKey(food)) {
            throw new NoSuchFoodException("No such food type: " + food.getName() + ".");
        }
        int current = stock.get(food);
        if (current > 0) {
            stock.put(food, current - 1);
            return true;
        }
        return false; // Stock déjà à 0
    }
}
