public class Example {
    public static void main ( String [] args ) {
        Canary titi = new Canary (" Titi ");
        Shark willy = new Shark (" Willy "); // Yes Willy is a shark here !
        willy . status () ;
        willy . smellBlood ( true );
        willy . status () ;
        titi . layEgg () ;
        System . out . println ( titi . getEggsCount () );
    }
}