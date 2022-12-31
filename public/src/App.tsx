import React from 'react';
import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="flex relative flex-col items-center justify-center py-20 bg-gray-900 w-screen min-h-screen">
      
      <div className="flex group flex-row items-center justify-center relative">
        <div 
        className={`
            absolute w-full h-full bg-pink-400 rounded-md -inset-0.5 
            bg-gradient-to-r from-pink-500 to-purple-500
            blur-sm
            opacity-75
            group-hover:opacity-100
            transition-opacity
            duration-1000
            ease-in-out
            animate-tilt
            
        `}
        ></div>
        <button
          onClick={()=>{
            window.open("https://github.com/porkytheblack")
          }}
        className="flex relative rounded-md cursor-pointer flex-row items-center justify-center px-4 py-2 bg-black divide-x ">
          <span className='text-white pr-4' >
            I 💖 Rust
          </span>
          <span className='text-white pl-4' >
            Checkout my stuff &rarr;
          </span>
        </button>
      </div>
      
      <div className="flex flex-col mt-5 w-4/5 lg:w-1/2 items-center justify-start">
        <div className='relative mb-5 flex w-full flex-row items-center justify-center'  >
          <div className='absolute -mb-5 w-full h-full bg-pink-400  inset-1'></div>
          <div className=' relative flex items-center justify-center w-full flex-row px-4 py-2 bg-black' >
            <span className='text-purple-800 text-2xl font-bold ' >
              A Poem 😁
            </span>
            
          </div>
        </div>
        <span className='text-justify text-white text-md font-semibold' >
        Rust, the systems programming language
        With a focus on safety, concurrency, and speed
        It's memory-efficient and statically-typed
        And offers low-level control and high-level abstractions
        <br/>
        <br/>
        
        From web servers to operating systems
        Rust can build it all
        With its powerful syntax and efficient runtime
        It's a language that stands tall
        <br/>
        <br/>
        Whether you're a beginner or a seasoned pro
        Rust has something for everyone to explore
        So give it a try and see what it can do
        I'm sure you'll love it more and more
        </span>
      </div>

      <div className="flex flex-row items-center w-4/5 justify-center mt-5">
        <span className='text-white' >
          Built with 💖 and ⚡ by 
        </span>
        <a className='text-blue-600 ml-5' href="https://github.com/porkytheblack">porkytheblack</a>
      </div>
    </div>
  );
}

export default App;
