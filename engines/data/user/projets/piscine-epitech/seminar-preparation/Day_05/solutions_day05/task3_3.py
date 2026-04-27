### NESTED LIST ###

superheroes = {
    "Batman" : {
        "id": 1,
        "aliases": ["Bruce Wayne", "Dark knight"],
        "location": {
            "number" : 1007,
            "street": "Mountain Drive",
            "city": "Gotham"
            }
        },
    "Superman" : {
        "id": 2,
        "aliases": ["Kal-El", "Clark Kent", "The Man of Steel"],
        "location": {
            "number" : 344,
            "street": "Clinton Street",
            "apartment": "3D",
            "city": "Metropolis"
            }
        },
    }

### SPECIFIC OUTPUT ###

new_aliase = "Caped Crusader"
superheroes["Batman"]["aliases"].append(new_aliase)
print(superheroes["Batman"]["aliases"])
