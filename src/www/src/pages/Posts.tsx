import { useEffect, useState } from 'react';
import Navbar from '../components/Navbar';
import PostsList from '../components/PostsList';
import { loadPosts } from '../utils/content';
import type { Post } from '../types/content';

function Posts() {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchPosts = async () => {
      try {
        const posts = await loadPosts();
        setPosts(posts);
      } catch (error) {
        console.error('Error loading posts:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchPosts();
  }, []);

  return (
    <div className="container mx-auto px-4">
      <Navbar />

      <div className="mt-8">
        {loading ? (
          <div className="text-center py-12">
            <p className="text-lg text-gray-600 dark:text-gray-400">
              Loading posts...
            </p>
          </div>
        ) : (
          <PostsList posts={posts} />
        )}
      </div>
    </div>
  );
}

export default Posts;
