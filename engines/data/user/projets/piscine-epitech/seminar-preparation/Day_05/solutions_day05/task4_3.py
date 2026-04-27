### SAMPLE MEETINGS DATA ###

meetings = [
    ["Monday", "3:30 PM", "Joe", "Sam"],
    ["Monday", "4:30 PM", "Bob", "Alice"],
    ["Tuesday", "3:30 PM", "Joe", "Bob", "Alice", "Sam"],
    ["Tuesday", "9:30 AM", "Joe", "Bob"]
]

### INPUT NAME ###

name = input("Enter the name of the person: ").strip()

### SEARCH FOR MEETINGS ###

print(f"\nMeetings involving {name}:")
found = False

for meeting in meetings:
    day = meeting[0]
    time = meeting[1]
    participants = meeting[2:]
     
    if name in participants:
        print(f"- {day} at {time}")
        found = True

### OUTPUT IF NONE FOUND ###

if not found:
    print("No meetings found for this person.")
