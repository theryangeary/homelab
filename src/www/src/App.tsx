import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import headshot from '/headshot.jpg'
import './App.css'
import Projects from './pages/Projects'
import Posts from './pages/Posts'
import PostPage from './pages/PostPage'

function Home() {
  return (
    <div className="container mx-auto px-4 flex h-screen">
      <div className="m-auto">
          <h1 className="font-bold underline p-4 text-primary">
              <span className="text-4xl md:text-6xl lg:text-8xl">
                  Ryan Geary
              </span>
          </h1>

          <p className="flex justify-center text-secondary">Software Engineer @Lyft</p>
          <p className="flex justify-center text-secondary">FOSS Developer</p>

          <div className="flex justify-center">
            <img src={headshot} alt="a pic of me" className="w-3xs rounded-full p-10 " />
          </div>

          <div className="grid grid-cols-2 grid-rows-2 gap-4">
            {[
                ['Projects', '/projects', '_self'],
                ['Posts', '/posts', '_self'],
                ['GitHub', 'https://github.com/theryangeary/', '_blank'],
                ['LinkedIn', 'https://www.linkedin.com/in/theryangeary/', '_blank'],
              ].map(([title, url, target]) => (
                <Link to={url} className="
                        flex
                        justify-center
                        text-amber-200
                        hover:text-amber-50
                        bg-purple-900/75
                        p-4
                        border-4
                        border-purple-300
                        rounded-none
                        active:translate-1
                        active:shadow-none
                        shadow-[8px_8px_0_rgba(0,0,0,0.25)]
                " target={target}>{title}</Link>
            ))}
          </div>
      </div>
    </div>
  )
}

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/projects" element={<Projects />} />
        <Route path="/posts" element={<Posts />} />
        <Route path="/posts/:slug" element={<PostPage />} />
      </Routes>
    </Router>
  )
}

export default App
