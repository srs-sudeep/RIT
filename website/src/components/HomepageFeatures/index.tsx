import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  emoji: string;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Educational',
    emoji: '📚',
    description: (
      <>
        Learn how Git works internally by building it yourself. Understand
        content-addressable storage, DAGs, and version control fundamentals.
      </>
    ),
  },
  {
    title: 'Rust-Powered',
    emoji: '🦀',
    description: (
      <>
        Built with Rust for performance and safety. Demonstrates systems
        programming concepts including file I/O, hashing, and compression.
      </>
    ),
  },
  {
    title: 'Well Documented',
    emoji: '📖',
    description: (
      <>
        Comprehensive documentation with examples, architecture guides, and
        command references. Perfect for learning and contributing.
      </>
    ),
  },
  {
    title: 'Open Source',
    emoji: '🌟',
    description: (
      <>
        MIT licensed and open source. Feel free to use, modify, and learn from
        this implementation.
      </>
    ),
  },
];

function Feature({title, emoji, description}: FeatureItem) {
  return (
    <div className={clsx('col col--3')}>
      <div className="text--center">
        <div style={{fontSize: '3rem', marginBottom: '1rem'}}>{emoji}</div>
        <Heading as="h3">{title}</Heading>
      </div>
      <div className="text--center padding-horiz--md">
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
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
