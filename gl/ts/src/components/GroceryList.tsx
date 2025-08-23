import {
  closestCenter,
  CollisionDetection,
  DndContext,
  DragOverEvent,
  DragOverlay,
  DragStartEvent,
  getFirstCollision,
  KeyboardSensor,
  MouseSensor,
  pointerWithin,
  rectIntersection,
  TouchSensor,
  UniqueIdentifier,
  useSensor,
  useSensors,
  type DragEndEvent
} from '@dnd-kit/core'
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy
} from '@dnd-kit/sortable'
import { ReactNode, useCallback, useRef, useState } from 'react'
import { CategoryRepository, getLabel as getCategoryLabel } from '../hooks/useCategories'
import { GroceryListRepository } from '../hooks/useGroceryList'
import { Category as CategoryModel } from '../types/category'
import { GroceryListEntry } from '../types/grocery'
import Category from './Category'
import GroceryItem from './GroceryItem'
import SortableCategory from './SortableCategory'

type Items = Record<UniqueIdentifier, UniqueIdentifier[]>;

interface GroceryListProps {
  groceryListRepository: GroceryListRepository
  categoryRepository: CategoryRepository
}

export default function GroceryList({
  groceryListRepository,
  categoryRepository,
}: GroceryListProps) {
  if (groceryListRepository.loading || categoryRepository.loading) {
    return <div className="text-center">Loading...</div>
  }

  categoryRepository.categories.sort((a, b) => a.position - b.position);

  const sensors = useSensors(
    useSensor(MouseSensor),
    useSensor(TouchSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const [items, setItems] = useState<Items>(
    () => {
      var result: Items = {}
      categoryRepository.categories.forEach((category) => {
        result[getCategoryLabel(category)] = groceryListRepository.entries
          .filter((entry) => entry.category_id === category.id)
          .map((entry) => `entry-${entry.id}`)
      })
      return result;
    }
  );
  const [clonedItems, setClonedItems] = useState<Items>(items);

  const [containers, setContainers] = useState(
    Object.keys(items) as UniqueIdentifier[]
  );

  const [activeId, setActiveId] = useState<UniqueIdentifier | null>(null);

  const lastOverId = useRef<UniqueIdentifier | null>(null);
  const recentlyMovedToNewContainer = useRef(false);
  // const isSortingContainer =
  //   activeId != null ? containers.includes(activeId) : false;

  const collisionDetectionStrategy: CollisionDetection = useCallback(
    (args) => {
      if (activeId && activeId in items) {
        return closestCenter({
          ...args,
          droppableContainers: args.droppableContainers.filter(
            (container) => container.id in items
          ),
        });
      }

      // Start by finding any intersecting droppable
      const pointerIntersections = pointerWithin(args);
      const intersections =
        pointerIntersections.length > 0
          ? // If there are droppables intersecting with the pointer, return those
          pointerIntersections
          : rectIntersection(args);
      let overId = getFirstCollision(intersections, 'id');

      if (overId != null) {
        if (overId in items) {
          const containerItems = items[overId];

          if (containerItems.length > 0) {
            // Return the closest droppable within that container
            overId = closestCenter({
              ...args,
              droppableContainers: args.droppableContainers.filter(
                (container) =>
                  container.id === overId ||
                  containerItems.includes(container.id)
              ),
            })[0]?.id;
          }
        }

        lastOverId.current = overId;
        return [{ id: overId }];
      }

      // When a draggable item moves to a new container, the layout may shift
      // and the `overId` may become `null`. We manually set the cached `lastOverId`
      // to the id of the draggable item that was moved to the new container, otherwise
      // the previous `overId` will be returned which can cause items to incorrectly shift positions
      if (recentlyMovedToNewContainer.current) {
        lastOverId.current = activeId;
      }

      // If no droppable is matched, return the last match
      return lastOverId.current ? [{ id: lastOverId.current }] : [];
    },
    [activeId, items]
  );

  const findContainer = (id: UniqueIdentifier) => {
    if (id in items) {
      return id;
    }

    return Object.keys(items).find((key) => items[key].includes(id));
  };

  function handleDragStart(event: DragStartEvent) {
    setActiveId(event.active.id);
    setClonedItems(items);
  }

  const handleDragEnd = (event: DragEndEvent) => {
    const active = event.active;
    const over = event.over;

    var newCategoryId = undefined;
    var newPosition: number | undefined = 0;

    const movingContainers = active.id in clonedItems && over?.id;
    if (movingContainers) {
      const activeIndex = containers.indexOf(active.id);
      const overIndex = containers.indexOf(over.id);

      setContainers((containers) => {
        return arrayMove(containers, activeIndex, overIndex);
      });

      const draggedCategory: CategoryModel = active.data.current?.category;
      categoryRepository.reorderCategories(draggedCategory.id, over?.data.current?.category.position);
    }

    const activeContainer = findContainer(active.id);

    if (!activeContainer) {
      setActiveId(null);
      return;
    }

    const overId = over?.id;

    if (overId == null) {
      setActiveId(null);
      return;
    }

    const overContainer = findContainer(overId);

    if (overContainer && !movingContainers) {
      const activeIndex = items[activeContainer].indexOf(active.id);
      const overIndex = items[overContainer].indexOf(overId);
      const itemAtDropLocationWhenDragStart = clonedItems[overContainer][overIndex];

      newCategoryId = categoryRepository.getByLabel(overContainer as string)?.id;

      // if nothing was at this index before we are appending to the end
      if (itemAtDropLocationWhenDragStart === undefined) {
        // set newPosition to 1 after the last item in the drop container
        const lastPosition = groceryListRepository.getByLabel(clonedItems[overContainer].slice(-1)[0] as string)?.position
        newPosition = 1 + (lastPosition ? lastPosition : 0)
      } else {
        // there was something at this index before, use that items position
        newPosition = groceryListRepository.getByLabel(itemAtDropLocationWhenDragStart as string)?.position
      }
      
      groceryListRepository.reorderEntries(active.data.current?.entry.id, newPosition, newCategoryId);

      if (activeIndex !== overIndex) {
        setItems((items) => ({
          ...items,
          [overContainer]: arrayMove(
            items[overContainer],
            activeIndex,
            overIndex
          ),
        }));
      }
    }

    setActiveId(null);
  }

  function handleDragOver({ active, over }: DragOverEvent) {
    const overId = over?.id;
    if (!overId) { return }

    const overContainer = findContainer(overId);
    const activeContainer = findContainer(active.id);

    if (!overContainer || !activeContainer) {
      return;
    }

    if (activeContainer !== overContainer) {
      setItems((items) => {
        const activeItems = items[activeContainer];
        const overItems = items[overContainer];
        const overIndex = overItems.indexOf(overId);
        const activeIndex = activeItems.indexOf(active.id);

        let newIndex: number;

        if (overId in items) {
          newIndex = overItems.length + 1;
        } else {
          const isBelowOverItem =
            over &&
            active.rect.current.translated &&
            active.rect.current.translated.top >
            over.rect.top + over.rect.height;

          const modifier = isBelowOverItem ? 1 : 0;

          newIndex =
            overIndex >= 0 ? overIndex + modifier : overItems.length + 1;
        }

        recentlyMovedToNewContainer.current = true;

        return {
          ...items,
          [activeContainer]: items[activeContainer].filter(
            (item) => item !== active.id
          ),
          [overContainer]: [
            ...items[overContainer].slice(0, newIndex),
            items[activeContainer][activeIndex],
            ...items[overContainer].slice(
              newIndex,
              items[overContainer].length
            ),
          ],
        };
      });
    }
  }

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={collisionDetectionStrategy}
      onDragStart={handleDragStart}
      onDragEnd={handleDragEnd}
      onDragOver={handleDragOver}
    >
      <SortableContext items={containers} strategy={verticalListSortingStrategy}>
        {
          containers.map((categoryLabel) => {
            const category = categoryRepository.getByLabel(categoryLabel as string);
            const glitems: GroceryListEntry[] = items[categoryLabel].map((label: UniqueIdentifier) => groceryListRepository.getByLabel(label as string)).filter((t) => t !== undefined)
            if (category) {
              return <SortableCategory key={categoryLabel} category={category} items={glitems} groceryListRepository={groceryListRepository} />
            }
          })
        }
      </SortableContext>

      <DragOverlay dropAnimation={{
        duration: 500,
        easing: 'cubic-bezier(0.18, 0.67, 0.6, 1.22)',
      }}>
        {activeId
          ? containers.includes(activeId)
            ? renderContainerDragOverlay(activeId)
            : renderSortableItemDragOverlay(activeId)
          : null}
      </DragOverlay>
    </DndContext>
  )

  function renderSortableItemDragOverlay(id: UniqueIdentifier): React.ReactNode {
    {
      const item = groceryListRepository.getByLabel(id as string);
      return item ? (
        <GroceryItem
          item={item}
          onDelete={groceryListRepository.deleteEntry}
          onFetchSuggestions={groceryListRepository.fetchSuggestions}
          onUpdate={groceryListRepository.updateEntry}
          dragHandleProps={undefined} />
      ) : null
    }
  }

  function renderContainerDragOverlay(containerId: UniqueIdentifier): ReactNode {
    {
      const c = categoryRepository.getByLabel(containerId as string);
      const glitems: GroceryListEntry[] = items[containerId].map((label: UniqueIdentifier) => groceryListRepository.getByLabel(label as string)).filter((t) => t !== undefined)
      return c ?
        <Category category={c} items={glitems} groceryListRepository={groceryListRepository} /> : null
    }
  }

}
