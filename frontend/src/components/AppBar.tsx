import { FC } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import dynamic from "next/dynamic";

const WalletMultiButtonDynamic = dynamic(
    async () => (await import('@solana/wallet-adapter-react-ui')).WalletMultiButton,
    { ssr: false }
);

export const AppBar: FC = () => {
    return (
        <div className="flex w-full justify-end">
            <WalletMultiButtonDynamic />
        </div>
    );
};
