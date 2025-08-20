import { SortableContext } from '@dnd-kit/sortable';
import { GroceryListRepository } from '../hooks/useGroceryList';
import { Category as CategoryModel } from '../types/category';
import SortableGroceryItem from './SortableGroceryItem';

interface CategoryProps {
    category: CategoryModel,
    groceryListRepository: GroceryListRepository,
    dragHandleProps?: any,
}

export default function Category({
    category,
    groceryListRepository,
    dragHandleProps,
}: CategoryProps) {
    const items = groceryListRepository.entries.filter(entry => entry.category_id === category.id);
    const itemIds = items.map((entry) => `entry-${entry.id}`);

    return (
        <div>
            <div className="flex bg-sky-400/50 text-black font-bold p-1">
                <div
                    {...dragHandleProps}
                    className="cursor-grab active:cursor-grabbing text-white-1000 hover:text-gray-600 px-1"
                    title="Drag to reorder"
                >
                    ⋮⋮
                </div>
                <p className="pl-2">{category.name}</p>
            </div>
            <SortableContext items={itemIds}>
                <div>
                    {items.map((entry) => (
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
