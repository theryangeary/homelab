import { Link } from 'react-router-dom';
import type { Post } from '../types/content';

interface PostsListProps {
  posts: Post[];
}

interface PostCardProps {
  post: Post;
}

function PostCard({ post }: PostCardProps) {
  const formattedDate = new Date(post.date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });

  return (
    <article className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
      <header className="mb-4">
        <h2 className="text-xl font-semibold text-violet-900/50 dark:text-violet-300 mb-2">
          <Link to={`/posts/${post.slug}`} className="hover:underline">
            {post.title}
          </Link>
        </h2>
        <div className="flex flex-wrap items-center gap-4 text-sm text-gray-600 dark:text-gray-400">
          <time dateTime={post.date}>{formattedDate}</time>
          <div className="flex gap-2">
            {post.tags.map((tag) => (
              <span
                key={tag}
                className="bg-violet-100 dark:bg-violet-900/30 text-violet-800 dark:text-violet-300 px-2 py-1 rounded text-xs"
              >
                #{tag}
              </span>
            ))}
          </div>
        </div>
      </header>
      
      <p className="text-gray-700 dark:text-gray-300 mb-4">
        {post.excerpt}
      </p>
      
      <Link 
        to={`/posts/${post.slug}`}
        className="text-violet-600 dark:text-violet-400 hover:underline font-medium"
      >
        Read more →
      </Link>
    </article>
  );
}

function PostsList({ posts }: PostsListProps) {
  if (posts.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-lg text-gray-600 dark:text-gray-400">
          No posts found.
        </p>
      </div>
    );
  }

  return (
    <div className="grid gap-6 md:gap-8">
      {posts.map((post) => (
        <PostCard key={post.slug} post={post} />
      ))}
    </div>
  );
}

export default PostsList;