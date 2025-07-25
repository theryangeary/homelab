import matter from 'gray-matter';
import type { Project, Post, PostFrontmatter } from '../types/content';

export async function loadProjects(): Promise<Project[]> {
  try {
    const response = await fetch('/projects.json');
    if (!response.ok) {
      throw new Error('Failed to load projects');
    }
    return await response.json();
  } catch (error) {
    console.error('Error loading projects:', error);
    return [];
  }
}

export async function loadPosts(): Promise<Post[]> {
  const postModules = import.meta.glob('/src/content/posts/*.md', { query: '?raw', import: 'default' });
  const posts: Post[] = [];

  for (const [path, modulePromise] of Object.entries(postModules)) {
    try {
      const content = await modulePromise() as string;
      const { data, content: markdownContent } = matter(content);
      const frontmatter = data as PostFrontmatter;

      const slug = path.replace('/src/content/posts/', '').replace('.md', '');

      posts.push({
        slug,
        title: frontmatter.title,
        date: frontmatter.date,
        tags: frontmatter.tags,
        excerpt: frontmatter.excerpt,
        content: markdownContent,
      });
    } catch (error) {
      console.error(`Error loading post ${path}:`, error);
    }
  }

  return posts.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
}

export async function loadPost(slug: string): Promise<Post | null> {
  // Load all posts and find the matching one
  const posts = await loadPosts();
  return posts.find(post => post.slug === slug) || null;
}
