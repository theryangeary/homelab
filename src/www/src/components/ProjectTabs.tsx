import { useState } from 'react';
import ProjectGrid from './ProjectGrid';
import type { Project } from '../types/content';

interface ProjectTabsProps {
  projects: Project[];
}

type TabType = 'production' | 'toy';

function ProjectTabs({ projects }: ProjectTabsProps) {
  const [activeTab, setActiveTab] = useState<TabType>('production');

  const productionProjects = projects.filter(project => project.category === 'production');
  const toyProjects = projects.filter(project => project.category === 'toy');

  const currentProjects = activeTab === 'production' ? productionProjects : toyProjects;

  return (
    <div className="space-y-6 divide-solid divide-purple-300 divide-y-1">
      <div className="flex justify-center">
        <button
          onClick={() => setActiveTab('production')}
          className={`px-6 py-3 border-1 border-purple-300 font-medium transition-colors ${
            activeTab === 'production'
              ? 'bg-purple-900/75 text-amber-200 '
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
          }`}
        >
          Production Projects
        </button>
        <button
          onClick={() => setActiveTab('toy')}
          className={`px-6 py-3 border-1 border-purple-300 font-medium transition-colors ${
            activeTab === 'toy'
              ? 'bg-purple-900/75 text-amber-200 '
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
          }`}
        >
          Toy Projects
        </button>
      </div>



      <div className="mt-8">
        <ProjectGrid projects={currentProjects} />
      </div>
    </div>
  );
}

export default ProjectTabs;
