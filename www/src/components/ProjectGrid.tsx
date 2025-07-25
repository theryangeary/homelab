import ProjectCard from './ProjectCard';
import type { Project } from '../types/content';

interface ProjectGridProps {
  projects: Project[];
}

function ProjectGrid({ projects }: ProjectGridProps) {
  if (projects.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-lg text-gray-600 dark:text-gray-400">
          No projects found.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      {projects.length > 0 && (
        <section>
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {projects.map((project) => (
              <ProjectCard key={project.id} project={project} />
            ))}
          </div>
        </section>
      )}
    </div>
  );
}

export default ProjectGrid;
