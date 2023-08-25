import React from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useBaseUrl from '@docusaurus/useBaseUrl';


// function Acknowledgements() {
//   return (
//     <Layout title="Acknowledgements">
//       <div className="container">
//         <h1>Acknowledgements</h1>
//         <p>
//           Scout is an open source vulnerability analyzer developed by <Link to="https://www.coinfabrik.com/">CoinFabrik's</Link> Research and Development team.
//         </p>
//         <h2>Web3 Foundation Grants</h2>
//         <p>
//           We received support through grants from the <Link to="https://github.com/w3f/Grants-Program/tree/master">Web3 Foundation Grants Program</Link>.
//         </p>
//         <table>
//           <thead>
//             <tr>
//               <th>Grant Program</th>
//               <th>Stage</th>
//               <th>Application Form</th> {/* Updated title */}
//               <th>Description</th>
//             </tr>
//           </thead>
//           <tbody>
//             <tr>
//               <td rowSpan="2">
//                 <img src="https://ityfuzz.assets.fuzz.land/web3.png" width="100" alt="Web3 Foundation" />
//               </td>
//               <td><Link to="https://github.com/CoinFabrik/web3-grant">Proof of Concept</Link></td>
//               <td><Link to="https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik.md">ScoutCoinFabrik</Link></td>
//               <td>During the Proof of Concept (PoC) stage, we collaborated with the Laboratory on Foundations and Tools for Software Engineering (<Link to="https://lafhis.dc.uba.ar/">LaFHIS</Link>) at the <Link to="https://www.uba.ar/internacionales/index.php?lang=en">University of Buenos Aires</Link> to establish analysis techniques and tools for our detectors, as well as to create an initial list of vulnerability classes and code examples.</td>
//             </tr>
//             <tr>
//               <td><Link to="https://coinfabrik.github.io/scout/">Prototype</Link></td>
//               <td><Link to="https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik_2.md">ScoutCoinFabrik_2</Link></td>
//               <td>We built a functioning prototype using linting detectors built with <Link to="https://github.com/trailofbits/dylint">Dylint</Link> and expanded the list of vulnerability classes, detectors, and test cases.</td>
//             </tr>
//           </tbody>
//         </table>
//         <h3>Aleph Grants Grants</h3>
//         <p>
//           We received support through grants from the <Link to="https://alephzero.org/ecosystem-funding-program">Aleph Zero Ecosystem Funding Program</Link>.
//         </p>
//         <table>
//           [COMPLETE_HERE]
//         </table>
//       </div>
//     </Layout>
//   );
// }

// export default Acknowledgements;




// function Acknowledgements() {
//   return (
//     <Layout title="Acknowledgements">
//       <div className="container">
//         <h1>Acknowledgements</h1>
//         <p>
//           Scout is an open source vulnerability analyzer developed by <Link to="https://www.coinfabrik.com/">CoinFabrik's</Link> Research and Development team.
//         </p>
//         <h2>Web3 Foundation Grants</h2>
//         <p>
//           We received support through grants from the <Link to="https://github.com/w3f/Grants-Program/tree/master">Web3 Foundation Grants Program</Link>.
//         </p>
//         <table>
//           <thead>
//             <tr>
//               <th>Grant Program</th>
//               <th>Stage</th>
//               <th>Application Form</th> {/* Updated title */}
//               <th>Description</th>
//             </tr>
//           </thead>
//           <tbody>
//             <tr>
//               <td rowSpan="2">
//                 <img src="https://ityfuzz.assets.fuzz.land/web3.png" width="100" alt="Web3 Foundation" />
//               </td>
//               <td><Link to="https://github.com/CoinFabrik/web3-grant">Proof of Concept</Link></td>
//               <td><Link to="https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik.md">ScoutCoinFabrik</Link></td>
//               <td>During the Proof of Concept (PoC) stage, we collaborated with the Laboratory on Foundations and Tools for Software Engineering (<Link to="https://lafhis.dc.uba.ar/">LaFHIS</Link>) at the <Link to="https://www.uba.ar/internacionales/index.php?lang=en">University of Buenos Aires</Link> to establish analysis techniques and tools for our detectors, as well as to create an initial list of vulnerability classes and code examples.</td>
//             </tr>
//             <tr>
//               <td><Link to="https://coinfabrik.github.io/scout/">Prototype</Link></td>
//               <td><Link to="https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik_2.md">ScoutCoinFabrik_2</Link></td>
//               <td>We built a functioning prototype using linting detectors built with <Link to="https://github.com/trailofbits/dylint">Dylint</Link> and expanded the list of vulnerability classes, detectors, and test cases.</td>
//             </tr>
//           </tbody>
//         </table>
//         <h3>Aleph Grants Grants</h3>
//         <p>
//           We received support through grants from the <Link to="https://alephzero.org/ecosystem-funding-program">Aleph Zero Ecosystem Funding Program</Link>.
//         </p>
//         <table>
//           <thead>
//             <tr>
//               <th>Grant Program</th>
//               <th>Description</th>
//             </tr>
//           </thead>
//           <tbody>
//             <tr>
//               <td>
//                 <img src="https://ityfuzz.assets.fuzz.land/web3.png" width="100" alt="Aleph Zero Grant Program" />
//               </td>
//               <td>We improved the precision and number of detectors for the tool with a multi-phase approach. This included a manual vulnerability analysis of projects in the Aleph Zero ecosystem, extensive testing of the tool on top projects, and refining detection accuracy.</td>
//             </tr>
//           </tbody>
//         </table>
//       </div>
//     </Layout>
//   );
// }

// export default Acknowledgements;


function Acknowledgements() {
  return (
    <Layout title="Acknowledgements">
      <div className="container">
        <h1>Acknowledgements</h1>
        <p>
          Scout is an open source vulnerability analyzer developed by <Link to="https://www.coinfabrik.com/">CoinFabrik's</Link> Research and Development team.
        </p>
        <h2>Grants</h2>
        <p>
          We received support through grants from both the <Link to="https://github.com/w3f/Grants-Program/tree/master">Web3 Foundation Grants Program</Link> and the <Link to="https://alephzero.org/ecosystem-funding-program">Aleph Zero Ecosystem Funding Program</Link>.
        </p>
        <table>
          <thead>
            <tr>
              <th>Grant Program</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>
                <img src={useBaseUrl('img/web3-foundation.png')} width="100" alt="Web3 Foundation" />
              </td>
              <td>
                <strong>Proof of Concept:</strong> We collaborated with the Laboratory on Foundations and Tools for Software Engineering (<Link to="https://lafhis.dc.uba.ar/">LaFHIS</Link>) at the <Link to="https://www.uba.ar/internacionales/index.php?lang=en">University of Buenos Aires</Link> to establish analysis techniques and tools for our detectors, as well as to create an initial list of vulnerability classes and code examples. <Link to="https://github.com/CoinFabrik/web3-grant">View PoC</Link> | <Link to="https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik.md">View Application Form</Link>.
                <br /><br />
                <strong>Prototype:</strong> We built a functioning prototype using linting detectors built with <Link to="https://github.com/trailofbits/dylint">Dylint</Link> and expanded the list of vulnerability classes, detectors, and test cases. <Link to="https://coinfabrik.github.io/scout/">View Prototype</Link> | <Link to="https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik_2.md">View Application Form</Link>.
              </td>
            </tr>
            <tr>
              <td>
                <img src={useBaseUrl('img/aleph-zero.png')} width="100" alt="Aleph Zero Grant Program" />
              </td>
              <td>
                We improved the precision and number of detectors for the tool with a multi-phase approach. This included a manual vulnerability analysis of projects in the Aleph Zero ecosystem, extensive testing of the tool on top projects, and refining detection accuracy.
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </Layout>
  );
}

export default Acknowledgements;




