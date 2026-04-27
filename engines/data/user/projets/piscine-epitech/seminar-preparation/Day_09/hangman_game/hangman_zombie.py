import random, datetime, sys, random, os

### CONSTANTS & EVENTS ###

max_pop = 1000
zombies = 0
infect_letter = 0.10
infect_word = 0.50

dramatic_events = [
    # STAGE 1 — WARNING
    " Strange radio silence in the capital…",
    " Rumors spread about a new infection…",
    " People panic-buy food and supplies…",
    " First hospitals report unusual cases…",
    "⚠️ Government issues first health alert…",

    # STAGE 2 — COLLAPSE
    " The military base collapsed!",
    " Hospitals are overflowing with the infected…",
    " Radio silence from London…",
    " Panic in New York, chaos everywhere!",
    " A scientist has been bitten during research!",
    " Evacuation failed, helicopter crashed!",
    " Governments collapsing one by one…",

    # STAGE 3 — HORROR
    " Experiments gone wrong, zombies mutating!",
    "️ Last radio DJ screams: 'Save yourselves!'",
    " Horde spotted near the Eiffel Tower!",
    " Blood floods the streets…",
    "☢️ Nuclear plant meltdown spreads toxic gas!",
    " Entire cities burned to contain outbreak!",
    " Mass graves overflow in the countryside…",
    "☠️ Humanity is on the brink of extinction…",
]

### LOADING WORDS ###

def load_words_from_argv():    
    if len(sys.argv) < 2:
        print("Error: missing argument")
        sys.exit(1)
    path = sys.argv[1]
    
    if not os.path.exists(path):
        print(f"Error: file '{path}' not found")
        sys.exit(1)
    
    words = []
    try:
        with open(path, "r", encoding="utf-8") as f:
            for line in f:
                w = line.strip()
                if w and w.replace("-", "").isalpha():
                    words.append(w)
    except Exception as e:
        print(f"Error: cannot read file '{path}' → {e}")
        sys.exit(1)
    
    if not words:
        print(f"Error: file '{path}' contains no valid words")
        sys.exit(1)
    
    return words

### INIT GAME ###

def init_game():
    words = load_words_from_argv()
    secret_word = random.choice(words).upper()
    mask = ["_"] * len(secret_word)
    return {
        "secret_word": secret_word,
        "mask": mask,
        "population": max_pop,
        "zombies": 0,
        "attempts": 0,
        "last_event": "",
        "finished": False,
        "win": False,
    }

### HELPERS INTERNES ###

def _event_for(zombies_count: int) -> str:
    idx = min(zombies_count // 100, len(dramatic_events) - 1)
    return dramatic_events[idx]

def _check_end(state: dict) -> None:
    if "_" not in state["mask"]:
        state["finished"] = True
        state["win"] = True
    if state["population"] <= 0:
        state["population"] = 0
        state["zombies"] = max_pop
        state["finished"] = True
        state["win"] = False

### GAMEPLAY - ACTIONS ###

def apply_letter(state: dict, letter: str) -> dict:
    """Joue une lettre (1 char). Renvoie un petit résumé {found, lost, event}."""
    if state.get("finished"):
        return {"found": False, "lost": 0, "event": ""}

    letter = (letter or "").strip().upper()
    if len(letter) != 1 or not letter.isalpha():
        return {"found": False, "lost": 0, "event": ""}

    found = False
    for i, ch in enumerate(state["secret_word"]):
        if ch == letter and state["mask"][i] == "_":
            state["mask"][i] = letter
            found = True

    lost = 0
    if not found:
        lost = int(max_pop * infect_letter)
        state["population"] -= lost
        state["zombies"] += lost
        state["last_event"] = _event_for(state["zombies"])

    state["attempts"] += 1
    _check_end(state)
    return {"found": found, "lost": lost, "event": state.get("last_event", "")}

def apply_word(state: dict, guess: str) -> dict:
    """Joue un mot complet. Renvoie {correct, lost, event}."""
    if state.get("finished"):
        return {"correct": False, "lost": 0, "event": ""}

    guess = (guess or "").strip().upper()
    if not guess:
        return {"correct": False, "lost": 0, "event": ""}

    if guess == state["secret_word"]:
        state["mask"] = list(state["secret_word"])
        state["attempts"] += 1
        _check_end(state)
        return {"correct": True, "lost": 0, "event": ""}

    lost = int(max_pop * infect_word)
    state["population"] -= lost
    state["zombies"] += lost
    state["last_event"] = _event_for(state["zombies"])
    state["attempts"] += 1
    _check_end(state)
    return {"correct": False, "lost": lost, "event": state.get("last_event", "")}

### INFO HELPERS ###

def progress_bar(state: dict, blocks: int = 10) -> str:
    if blocks <= 0:
        return ""
    humans_blocks = round((state["population"] / max_pop) * blocks)
    humans_blocks = max(0, min(blocks, humans_blocks))
    zombies_blocks = blocks - humans_blocks
    return "█" * humans_blocks + "-" * zombies_blocks

### BEST SCORE SYSTEM ###

def best_score_update(state: dict, path: str = "best_scores.txt") -> str:
    if not state.get("finished"):
        return ""

    record = None
    try:
        with open(path, "r", encoding="utf-8") as f:
            for line in f:
                parts = [p.strip() for p in line.split(",")]
                if len(parts) >= 2 and parts[1].isdigit():
                    att = int(parts[1])
                    record = att if record is None else min(record, att)
    except Exception:
        pass

    try:
        if record is None or state["attempts"] < record:
            msg = f'Best ever!!! You\'ve guessed "{state["secret_word"]}" in {state["attempts"]} attempts.'
            now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            with open(path, "a", encoding="utf-8") as f:
                f.write(f"{now}, {state['attempts']}, {state['secret_word']}\n")
            return msg
        else:
            return (f'You\'ve guessed "{state["secret_word"]}" in {state["attempts"]} attempts. '
                    f"The record is {record} attempts.")
    except Exception:
        return "Score save failed (file permission?)."

if __name__ == "__main__":
    st = init_game()
    print("SECRET:", st["secret_word"])
    print("MASK:  ", " ".join(st["mask"]))
