import Map "mo:base/HashMap";
import Hash "mo:base/Hash";
import Nat "mo:base/Nat";
import Iter "mo:base/Iter";
import Text "mo:base/Text";

actor Assitant {
    type ToDo = {
        description: Text;
        completed: Bool;
    }

    // HashMap
    func natHash(n: Nat) : Hash.hash {
        Text.hash(Nat.toText(n));
    }

    //let -- immutable data type
    //var -- mutable data type

    var todos = Map.HashMap<Nat, ToDo>(0, Nat.equal, natHash);
    var nextId : Nat = 0;

    public query func getTodos() : async [ToDo] {
      Iter.toArray(todos.vals());
    };

    // ID ToDo Assignment
    public query func addTodo(description: Text) : async Nat {
        let id = nextId;
        todos.put(id, {description = description; completed = false;});
        nextId += 1;
        id;
    };

    // Update Assignment
    public func completeTodo(id: Nat) : async () {
        ignore do ? {
            let description = todo.get(id)!.description;
            todos.put(id, {description; completed = true;});
        }
    };


    public query func showTodos() : async Text {
        var output : Text = "\n___TO-DOs___\n";
        for(todo: ToDo in todos.vals()) {
          output #= "\n" # todo.description;
          if (todo.completed) {
            output #= "[OK]";
          }
        };

        output # "\n";
    };

    public func clearCompleted() : async () {
      todos := Map.mapFilter<Nat, Todo, ToDo>(todos, Nat.equal, natHash, func(_, todo) {
        if (todo.completed) null;
        else ?todo;
      });
    }
}