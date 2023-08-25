import React from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

function About() {
  return (
    <Layout title="About">
      <div className="container">
        <h1>About</h1>
        <p>
            We - <Link to="https://www.coinfabrik.com/r">CoinFabrik</Link> - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, and TEAL.
            <br/>
            <br/>
            Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.
        </p>
      </div>
    </Layout>
  );
}

export default About;
