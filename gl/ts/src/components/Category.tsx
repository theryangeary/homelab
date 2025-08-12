import { useDroppable } from '@dnd-kit/core';
import { SortableContext, verticalListSortingStrategy } from '@dnd-kit/sortable';
import { GroceryListRepository } from '../hooks/useGroceryList';
import { Category as CategoryModel } from '../types/category';
import SortableGroceryItem from './SortableGroceryItem';

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
    });

    return (
        <div ref={setNodeRef}>
            <div className="bg-sky-500"><p>{category.name}</p></div>
            <SortableContext items={groceryListRepository.entries.filter(entry => entry.category_id === category.id).map(entry => entry.id)} strategy={verticalListSortingStrategy}>
                <div className="space-y-2">
                    {groceryListRepository.entries.map((entry) => (
                        <SortableGroceryItem
                            key={entry.id}
                            entry={entry}
                            onUpdate={groceryListRepository.updateEntry}
                            onDelete={groceryListRepository.deleteEntry}
                            onFetchSuggestions={groceryListRepository.fetchSuggestions}
                        />
                    ))}
                </div>
            </SortableContext>
        </div>
    );
}
