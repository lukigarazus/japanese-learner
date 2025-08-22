import React from "react";

type AnnotatedTextProps = {
  text: string;
  annotations: Record<string, string>;
  className?: string;
};

const AnnotatedText: React.FC<AnnotatedTextProps> = ({
  text,
  annotations,
  className,
}) => {
  return (
    <span className={`[line-height:1] align-middle ${className ?? ""}`}>
      {Array.from(text).map((char, i) => {
        const annotation = annotations[char];
        if (annotation) {
          return (
            <ruby key={i} className="">
              {char}
              <rt className="">{annotation}</rt>
            </ruby>
          );
        }
        return <ruby key={i}>{char}</ruby>;
      })}
    </span>
  );
};

export default AnnotatedText;
