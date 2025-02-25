"use client";

import { useState, useEffect } from "react";

import { Button } from "@/components/ui/button";
import Wrapper from "@/components/wrapper";
import { firstImage, localAssetUrl } from "@/lib/tauri";

export default function Home() {
  const [imagePath, setImagePath] = useState<string | null>(null);
  const [subdir, setSubdir] = useState<string>('Downloads');

  useEffect(() => {
    firstImage(subdir).then(path => {
      if (path) {
        setImagePath(localAssetUrl(path));
      } else {
        setImagePath(null);
      }
    });
  }, [subdir]);

  return (
    <section className="flex flex-col lg:flex-row">
      <section className="flex h-screen w-full flex-col justify-between p-9 lg:h-auto">
        <Wrapper>
          <div className="mx-auto flex max-w-sm flex-col justify-between">
            <span
              className={`-mt-14 inline-block text-[64px] font-bold text-black dark:text-white`}
            >
              Asset
            </span>
            <p className="pb-6 font-medium">
              Render first JPG in your $HOME/{subdir} folder
            </p>

            <div>
              <Button size="xl" className="w-full font-bold" variant="brand" onClick={() => setSubdir('Downloads')}>
                In Downloads
              </Button>
            </div>
              <br />
            <div>
              <Button size="xl" className="w-full font-bold" variant="brand" onClick={() => setSubdir('Pictures')}>
                In Pictures
              </Button>
            </div>
          </div>
        </Wrapper>
      </section>

      {/* second half */}

        <section className="hidden h-screen w-full flex-col items-center justify-center bg-[#e0f5ff] p-9 lg:flex">
          {imagePath ? (
            <img src={imagePath} />
          ) : <p>Not found</p>}
        </section>
    </section>
  );
}
