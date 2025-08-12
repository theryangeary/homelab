import { useDroppable } from '@dnd-kit/core';
import { GroceryListRepository } from '../hooks/useGroceryList';
import { Category as CategoryModel } from '../types/category';
import GroceryItem from './GroceryItem';

interface CategoryProps {
    category: CategoryModel,
    groceryListRepository: GroceryListRepository,
}

export default function Category({
    category,
    groceryListRepository,
}: CategoryProps) {
    const { setNodeRef } = useDroppable({
        id: `category-${category.id}`,
        data: {
            category: category,
        }
    });

    const items = groceryListRepository.entries.filter(entry => entry.category_id === category.id);

    return (
        <div ref={setNodeRef}>
            <div className="bg-sky-500"><p>{category.name}</p></div>
                <div className="space-y-2">
                    {items.map((entry) => (
                        <GroceryItem
                            key={entry.id}
                            item={entry}
                            onUpdate={groceryListRepository.updateEntry}
                            onDelete={groceryListRepository.deleteEntry}
                            fetchSuggestions={groceryListRepository.fetchSuggestions}
                        />
                    ))}
                </div>
        </div>
    );
}
