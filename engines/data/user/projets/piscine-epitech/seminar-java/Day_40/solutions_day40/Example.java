public class Example {
    public static void main ( String args []) {
    Breakfast < AppleSmoothie , SoftBread > breakfast = new Breakfast < >( new AppleSmoothie
    () , new SoftBread () );
    Food food = new Cookie () ;
    Stock stock = new Stock () ;
    CustomerOrder order = new CustomerOrder ( stock );
    try {
    order . addItem ( food );
    order . addMenu ( breakfast );
    } catch ( NoSuchFoodException e) {
    System . out . println (e. getMessage () );
    }
    order . printOrder () ;
    }
    }