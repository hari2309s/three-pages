import { ReactNode } from "react";
import { Header } from "@/components/layout/Header";
import { AnimatedContainer } from "@three-pages/ui";

interface LayoutProps {
  children: ReactNode;
}

const Layout = ({ children }: LayoutProps) => {
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
    </AnimatedContainer>
  );
};

export default Layout;
