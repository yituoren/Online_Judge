import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import CreateJob from './components/CreateJob';
import GetJob from './components/GetJob';
import GetContest from './components/Contest';
import './App.css';

const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState('submit');

  return (
    <Router>
      <div className="App">
        <div className="sidebar">
          <h2>MY OJ</h2>
          <ul>
            <li onClick={() => setActiveTab('submit')}>
              <Link to="/">SUBMIT</Link>
            </li>
            <li onClick={() => setActiveTab('job')}>
              <Link to="/job">JOB</Link>
            </li>
            <li onClick={() => setActiveTab('contest')}>
              <Link to="/contest">CONTEST</Link>
            </li>
            <li onClick={() => setActiveTab('user')}>
              <Link to="/user">USER</Link>
            </li>
          </ul>
        </div>
        <div className="main-content">
          <Routes>
            <Route path="/" element={<CreateJob />} />
            <Route path="/job" element={<GetJob />} />
            <Route path="/contest" element={<GetContest />} />
            <Route path="/user" element={<div className="empty-page">USER Page</div>} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;
