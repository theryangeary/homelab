import { Link } from 'react-router-dom'

function Navbar() {
  return (
    <nav className="m-4">
      <div className="flex justify-center divide-x-1 divide-purple-300 divide-solid ">
        <Link className="text-xl text-purple-900/50 dark:text-violet-300 hover:underline decoration-3 pr-3 pl-3 md:text-3xl flex-1 " to="/" >Ryan Geary</Link>
        <Link className="text-xl text-purple-900/50 dark:text-violet-300 hover:underline decoration-3 pr-3 pl-3 flex-none " to="/projects" >Projects</Link>
        <Link className="text-xl text-purple-900/50 dark:text-violet-300 hover:underline decoration-3 pr-3 pl-3 flex-none " to="/posts" >Posts</Link>
      </div>
    </nav>
  )
}

export default Navbar
