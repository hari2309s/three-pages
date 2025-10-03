import { Variants, Transition } from "framer-motion";

// Common easing curves
export const easing = {
  smooth: [0.25, 0.1, 0.25, 1] as [number, number, number, number],
  spring: [0.6, 0.05, 0.01, 0.9] as [number, number, number, number],
  snappy: [0.5, 0, 0.75, 0] as [number, number, number, number],
};

// Common durations
export const duration = {
  fast: 0.2,
  normal: 0.3,
  slow: 0.5,
  slower: 0.8,
};

// Common spring configurations
export const spring = {
  smooth: {
    type: "spring",
    stiffness: 100,
    damping: 25,
    restDelta: 0.01,
  },
  bouncy: {
    type: "spring",
    stiffness: 400,
    damping: 10,
  },
  gentle: {
    type: "spring",
    stiffness: 60,
    damping: 20,
  },
} as const;

// Page transition animations
export const pageVariants: Variants = {
  initial: {
    opacity: 0,
    y: 20,
    scale: 0.98,
  },
  in: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: {
      duration: duration.normal,
      ease: easing.smooth,
    },
  },
  out: {
    opacity: 0,
    y: -20,
    scale: 0.98,
    transition: {
      duration: duration.fast,
      ease: easing.smooth,
    },
  },
};

// Card animations
export const cardVariants: Variants = {
  hidden: {
    opacity: 0,
    y: 30,
    scale: 0.95,
  },
  visible: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: {
      duration: duration.normal,
      ease: easing.smooth,
    },
  },
  hover: {
    y: -5,
    scale: 1.02,
    transition: {
      duration: duration.fast,
      ease: easing.snappy,
    },
  },
  tap: {
    scale: 0.98,
    transition: {
      duration: duration.fast / 2,
      ease: easing.snappy,
    },
  },
};

// Container animations for sections
export const containerVariants: Variants = {
  hidden: {
    opacity: 0,
  },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.1,
      delayChildren: 0.05,
    },
  },
  exit: {
    opacity: 0,
    transition: {
      staggerChildren: 0.05,
      staggerDirection: -1,
    },
  },
};

// List item animations (for staggered lists)
export const listItemVariants: Variants = {
  hidden: {
    opacity: 0,
    x: -20,
  },
  visible: {
    opacity: 1,
    x: 0,
    transition: {
      duration: duration.normal,
      ease: easing.smooth,
    },
  },
};

// Modal/Dialog animations
export const modalVariants: Variants = {
  hidden: {
    opacity: 0,
    scale: 0.8,
    y: 100,
  },
  visible: {
    opacity: 1,
    scale: 1,
    y: 0,
    transition: {
      duration: duration.normal,
      ease: easing.smooth,
    },
  },
  exit: {
    opacity: 0,
    scale: 0.8,
    y: 100,
    transition: {
      duration: duration.fast,
      ease: easing.smooth,
    },
  },
};

// Overlay/Backdrop animations
export const overlayVariants: Variants = {
  hidden: {
    opacity: 0,
  },
  visible: {
    opacity: 1,
    transition: {
      duration: duration.fast,
    },
  },
  exit: {
    opacity: 0,
    transition: {
      duration: duration.fast,
    },
  },
};

// Button animations
export const buttonVariants: Variants = {
  idle: {
    scale: 1,
  },
  hover: {
    scale: 1.05,
    transition: {
      duration: duration.fast,
      ease: easing.snappy,
    },
  },
  tap: {
    scale: 0.95,
    transition: {
      duration: duration.fast / 2,
      ease: easing.snappy,
    },
  },
};

// Loading spinner variants
export const spinnerVariants: Variants = {
  animate: {
    rotate: 360,
    transition: {
      duration: 1,
      repeat: Infinity,
      ease: "linear",
    },
  },
};

// Text animation variants
export const textVariants: Variants = {
  hidden: {
    opacity: 0,
    y: 20,
  },
  visible: {
    opacity: 1,
    y: 0,
    transition: {
      duration: duration.normal,
      ease: easing.smooth,
    },
  },
};

// Slide animations (useful for panels, sidebars, etc.)
export const slideVariants = {
  left: {
    hidden: { x: "-100%", opacity: 0 },
    visible: { x: 0, opacity: 1 },
    exit: { x: "-100%", opacity: 0 },
  },
  right: {
    hidden: { x: "100%", opacity: 0 },
    visible: { x: 0, opacity: 1 },
    exit: { x: "100%", opacity: 0 },
  },
  up: {
    hidden: { y: "100%", opacity: 0 },
    visible: { y: 0, opacity: 1 },
    exit: { y: "100%", opacity: 0 },
  },
  down: {
    hidden: { y: "-100%", opacity: 0 },
    visible: { y: 0, opacity: 1 },
    exit: { y: "-100%", opacity: 0 },
  },
};

// Fade animations
export const fadeVariants: Variants = {
  hidden: {
    opacity: 0,
  },
  visible: {
    opacity: 1,
    transition: {
      duration: duration.normal,
      ease: easing.smooth,
    },
  },
  exit: {
    opacity: 0,
    transition: {
      duration: duration.fast,
      ease: easing.smooth,
    },
  },
};

// Scale animations
export const scaleVariants: Variants = {
  hidden: {
    scale: 0,
    opacity: 0,
  },
  visible: {
    scale: 1,
    opacity: 1,
    transition: {
      duration: duration.normal,
      ease: easing.smooth,
    },
  },
  exit: {
    scale: 0,
    opacity: 0,
    transition: {
      duration: duration.fast,
      ease: easing.smooth,
    },
  },
};

// Common transition configurations
export const transitions = {
  smooth: {
    duration: duration.normal,
    ease: easing.smooth,
  } as Transition,
  spring: spring.smooth as Transition,
  fast: {
    duration: duration.fast,
    ease: easing.snappy,
  } as Transition,
  slow: {
    duration: duration.slow,
    ease: easing.smooth,
  } as Transition,
};

// Helper function to create staggered animations
export const createStaggerContainer = (staggerDelay = 0.1) => ({
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: staggerDelay,
      delayChildren: 0.05,
    },
  },
});

// Helper function for enter/exit animations
export const createEnterExitVariants = (
  enter: Record<string, any> = { opacity: 1 },
  exit: Record<string, any> = { opacity: 0 },
  transition: Transition = transitions.smooth,
): Variants => ({
  initial: exit,
  animate: {
    ...enter,
    transition,
  },
  exit: {
    ...exit,
    transition,
  },
});
