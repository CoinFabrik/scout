import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'The Tool',
    Svg: require('@site/static/img/scout_tool.svg').default,
    description: (
      <>
        Scout is an extensible open-source tool intended to assist ink! smart contract developers and auditors detect common security issues and deviations from best practices.
      </>
    ),
  },
  {
    title: 'Security',
    Svg: require('@site/static/img/scout_security.svg').default,
    description: (
      <>
        This tool will help developers write secure and more robust smart contracts. Our interest in this project comes from our experience in manual auditing and our usage of comparable tools in other blockchains.

      </>
    ),
  },
  {
    title: 'Research',
    Svg: require('@site/static/img/scout_research.svg').default,
    description: (
      <>
        To improve coverage and precision, we persist in research efforts on static and dynamic analysis techniques. Find more about our ongoing research at our associated repository.
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
