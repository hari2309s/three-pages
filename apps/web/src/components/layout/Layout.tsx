import { ReactNode } from "react";
import { Header } from "./Header";
import { Footer } from "./Footer";
import { AnimatedContainer } from "../animated";

interface LayoutProps {
  children: ReactNode;
}

export const Layout = ({ children }: LayoutProps) => {
  return (
    <AnimatedContainer
      variant="container"
      staggerChildren={true}
      className="flex min-h-screen flex-col"
    >
      <AnimatedContainer variant="fade">
        <Header />
      </AnimatedContainer>
      <AnimatedContainer
        variant="fade"
        delay={0.1}
        className="flex-1 container mx-auto px-4 py-8"
      >
        <main>{children}</main>
      </AnimatedContainer>
      <AnimatedContainer variant="fade" delay={0.2}>
        <Footer />
      </AnimatedContainer>
    </AnimatedContainer>
  );
};
