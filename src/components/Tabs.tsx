import { useState } from "react";

export const Tabs = ({
  tabs,
}: {
  tabs: { name: string; content: () => React.ReactNode }[];
}) => {
  const [activeTab, setActiveTab] = useState(0);
  return (
    <div className="w-full h-full flex flex-col">
      <div className="flex w-full gap-1 p-1 pb-0 rounded-t-lg h-[50px]">
        {tabs.map((tab, idx) => (
          <div
            key={tab.name}
            className={`p-2 cursor-pointer rounded-t-lg transition font-medium ${
              activeTab === idx
                ? "bg-gray-200 text-gray-900"
                : "bg-white text-gray-700 hover:bg-gray-100"
            }`}
            onClick={() => setActiveTab(idx)}
          >
            {tab.name}
          </div>
        ))}
      </div>
      <div className="bg-gray-200 flex-1">
        {tabs[activeTab]?.content() ?? null}
      </div>
    </div>
  );
};
