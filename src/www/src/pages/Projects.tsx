import { useEffect, useState } from 'react';
import Navbar from '../components/Navbar';
import ProjectTabs from '../components/ProjectTabs';
import { loadProjects } from '../utils/content';
import type { Project } from '../types/content';

function Projects() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchProjects = async () => {
      try {
        const projectData = await loadProjects();
        setProjects(projectData);
      } catch (error) {
        console.error('Error loading projects:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchProjects();
  }, []);

  return (
    <div className="container mx-auto px-4">
     <Navbar />

      <div className="mt-8">
        {loading ? (
          <div className="text-center py-12">
            <p className="text-lg text-gray-600 dark:text-gray-400">
              Loading projects...
            </p>
          </div>
        ) : (
          <ProjectTabs projects={projects} />
        )}
      </div>
    </div>
  );
}

export default Projects
