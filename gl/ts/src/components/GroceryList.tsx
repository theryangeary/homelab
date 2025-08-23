import {
  closestCenter,
  CollisionDetection,
  DataRef,
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
import { useCallback, useRef, useState } from 'react'
import { CategoryRepository, getLabel } from '../hooks/useCategories'
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
        result[getLabel(category)] = groceryListRepository.entries
          .filter((entry) => entry.category_id === category.id)
          .map((entry) => `entry-${entry.id}`)
      })
      return result;
    }
  );

  const [containers, setContainers] = useState(
    Object.keys(items) as UniqueIdentifier[]
  );

  const [active, setActive] = useState<DataRef | null>(null);
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

          // If a container is matched and it contains items (columns 'A', 'B', 'C')
          if (containerItems.length > 0) {
            // Return the closest droppable within that container
            overId = closestCenter({
              ...args,
              droppableContainers: args.droppableContainers.filter(
                (container) =>
                  container.id !== overId &&
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

  // const getIndex = (id: UniqueIdentifier) => {
  //   const container = findContainer(id);

  //   if (!container) {
  //     return -1;
  //   }

  //   const index = items[container].indexOf(id);

  //   return index;
  // };



  function handleDragStart(event: DragStartEvent) {
    setActive(event.active.data);
    setActiveId(event.active.id);
  }

  const handleDragEnd = (event: DragEndEvent) => {
    const active = event.active;
    const over = event.over;

    if (active.id in items && over?.id) {
      setContainers((containers) => {
        const activeIndex = containers.indexOf(active.id);
        const overIndex = containers.indexOf(over.id);

        return arrayMove(containers, activeIndex, overIndex);
      });
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

    if (overContainer) {
      const activeIndex = items[activeContainer].indexOf(active.id);
      const overIndex = items[overContainer].indexOf(overId);

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
    setActive(null);

    var newCategoryId = undefined;
    var newPosition: number | undefined = 0;

    if (event.active.data.current?.type === 'entry') {
      const draggedEntry: GroceryListEntry = event.active.data.current?.entry;

      // if dropped on another entry, take that entry's position AND category
      if (event.over?.data.current?.type === 'entry') {
        const dropEntry: GroceryListEntry = event.over?.data.current?.entry
        newPosition = dropEntry.position;
        if (draggedEntry.category_id !== dropEntry.category_id) {
          newCategoryId = dropEntry.category_id
        }
        // if dropped on a category, take that category's id
      } else if (event.over?.data.current?.type === 'category') {
        const dropCategory: CategoryModel = event.over?.data.current?.category
        newCategoryId = dropCategory.id
      }

      groceryListRepository.reorderEntries(draggedEntry.id, newPosition, newCategoryId);
    } else if (event.active.data.current?.type === 'category') {
      const draggedCategory: CategoryModel = event.active.data.current?.category;

      // use a defined default value
      newPosition = draggedCategory.position;

      // if dropped on an entry, take that entry's category's position
      if (event.over?.data.current?.type === 'entry') {
        const dropEntry: GroceryListEntry = event.over?.data.current?.entry
        const categoryId = dropEntry.category_id;
        newPosition = categoryRepository.categories.filter((category) => category.id === categoryId)[0].position;
        // if dropped on a category, take that category's position
      } else if (event.over?.data.current?.type === 'category') {
        const dropCategory: CategoryModel = event.over?.data.current?.category
        newPosition = dropCategory.position
      } else {
        console.error("dropped on unsupported droppable");
        return
      }

      categoryRepository.reorderCategories(draggedCategory.id, newPosition);
    } else {
      console.log("type issue?")
    }

  }

  // todo this doesn't show the item in the correct place in a different category
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
            if (category) {
              return <SortableCategory key={categoryLabel} category={category} groceryListRepository={groceryListRepository} />
            }
          })
        }
      </SortableContext>

      <DragOverlay dropAnimation={{
        duration: 500,
        easing: 'cubic-bezier(0.18, 0.67, 0.6, 1.22)',
      }}>
        {active && active.current?.type === 'category' ? (
          <Category category={categoryRepository.categories.filter((cat) => cat.id === active.current?.category.id)[0]} groceryListRepository={groceryListRepository} />
        ) : null}
        {active && active.current?.type === 'entry' ? (
          <GroceryItem
            item={groceryListRepository.entries.filter((entry) => entry.id === active.current?.entry.id)[0]}
            onDelete={groceryListRepository.deleteEntry}
            onFetchSuggestions={groceryListRepository.fetchSuggestions}
            onUpdate={groceryListRepository.updateEntry}
            dragHandleProps={undefined} />
        ) : null}
      </DragOverlay>
    </DndContext>
  )
}
