import Console from './components/Console';
import GroceryList from './components/GroceryList';
import { useGroceryList } from './hooks/useGroceryList';

function App() {
  const groceryListRepository = useGroceryList();

  return (
    <div className="min-h-screen bg-white">
      <div className="max-w-md mx-auto p-4">
        <h1 className="text-2xl font-bold mb-6 text-center">Grocery List</h1>
        <Console
          groceryListRepository={groceryListRepository}
        />
        <GroceryList
          groceryListRepository={groceryListRepository}
        />
      </div>
    </div>
  )
}

export default App
