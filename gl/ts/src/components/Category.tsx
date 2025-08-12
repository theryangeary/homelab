import { AnimateLayoutChanges, defaultAnimateLayoutChanges, SortableContext, useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { GroceryListRepository } from '../hooks/useGroceryList';
import { Category as CategoryModel } from '../types/category';
import SortableGroceryItem from './SortableGroceryItem';

const animateLayoutChanges: AnimateLayoutChanges = (args) =>
    defaultAnimateLayoutChanges({ ...args, wasDragging: true });

interface CategoryProps {
    category: CategoryModel,
    groceryListRepository: GroceryListRepository,
}

export default function Category({
    category,
    groceryListRepository,
}: CategoryProps) {
    const items = groceryListRepository.entries.filter(entry => entry.category_id === category.id);
    const id = `category-${category.id}`;

    const {
        attributes,
        isDragging,
        listeners,
        setNodeRef,
        transition,
        transform,
    } = useSortable({
        id,
        data: {
            type: 'container',
            children: items,
            category: category,
        },
        animateLayoutChanges
    });

    const style = {
        transform: CSS.Transform.toString(transform),
        transition,
    };

    return (
        <div ref={setNodeRef} style={{
            ...style,
            transition,
            transform: CSS.Translate.toString(transform),
            opacity: isDragging ? 0.5 : undefined,
        }}
            {...attributes} {...listeners}
        >
            <div className="bg-sky-500">      <div
                {...listeners} {...attributes}
                className="cursor-grab active:cursor-grabbing text-white-1000 hover:text-gray-600 px-1"
                title="Drag to reorder"
            >
                ⋮⋮
            </div><p>{category.name}</p></div>
            <SortableContext items={items}>
                <div className="space-y-2">
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
