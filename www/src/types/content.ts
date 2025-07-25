export interface Project {
  id: string;
  title: string;
  description: string;
  techStack: string[];
  githubUrl?: string;
  tryItUrl?: string;
  category: 'production' | 'toy';
}

export interface Post {
  slug: string;
  title: string;
  date: string;
  tags: string[];
  excerpt: string;
  content: string;
}

export interface PostFrontmatter {
  title: string;
  date: string;
  tags: string[];
  excerpt: string;
}
