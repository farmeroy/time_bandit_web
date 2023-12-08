import Session from "@/utils/Session";

const BanditLayout = ({ children }: { children: React.ReactNode }) => {
  return <Session>{children}</Session>;
};

export default BanditLayout;
