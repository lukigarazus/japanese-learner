import { XMarkIcon } from "@heroicons/react/24/outline";
import {
  Dialog as RadixDialog,
  Trigger,
  Content,
  Overlay,
  Portal,
  Close,
} from "@radix-ui/react-dialog";

export const Dialog = ({
  content,
  trigger,
  open,
  onOpenChange,
}: {
  content: React.ReactNode;
  trigger: React.ReactNode;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}) => {
  return (
    <RadixDialog open={open} onOpenChange={onOpenChange}>
      <Trigger asChild>{trigger}</Trigger>
      <Portal>
        <Overlay className="fixed inset-0 bg-black/40 z-40 transition-opacity animate-fade-in" />
        <Content className="fixed left-1/2 top-1/2 z-50 max-h-[80vh] max-w-lg w-full -translate-x-1/2 -translate-y-1/2 rounded-xl bg-white p-6 shadow-2xl focus:outline-none animate-pop-in overflow-y-auto">
          {content}
          <Close className="absolute top-3 right-3 text-gray-400 hover:text-gray-700 text-2xl cursor-pointer">
            <XMarkIcon className="h-6 w-6" />
          </Close>
        </Content>
      </Portal>
    </RadixDialog>
  );
};
