"use client";

import Image from "next/image";
import robot from "@/public/images/robot.png";

import { Button } from "@/components/ui/button";
import Wrapper from "@/components/wrapper";
import { getGoogleAuthCode, sidecarSend } from "@/lib/tauri";

export default function Home() {
  return (
    <section className="flex flex-col lg:flex-row">
      <section className="bale flex h-screen w-full flex-col justify-between p-9 lg:h-auto">
        <Wrapper>
          <div className="mx-auto flex max-w-sm flex-col justify-between">
            <span
              className={`-mt-14 inline-block text-[64px] font-bold text-black dark:text-white`}
            >
              01
            </span>
            <p className="pb-6 font-medium">
              Think of Kaminari as that friend who shows up with snacks and
              already picked the movie, so all you have to do is sit back and
              enjoy the show. I handled all the basic configs for you. Enjoy.
            </p>

            <div className="">
              <Button size="xl" className="w-full font-bold" variant="brand" onClick={async () => {
                const { authCode, redirectUri } = await getGoogleAuthCode();

                // Use these values to authenticate the user on the backend
                console.log(authCode, redirectUri);
              }}>
                <a
                  href="https://github.com/lucky-chap/kaminari"
                  target="_blank"
                  rel="noreferrer"
                  className="pb-1 text-zinc-100 dark:text-zinc-800"
                >
                  Auth
                </a>{" "}
              </Button>
            </div>
            <Button onClick={async () => {
              await sidecarSend('log', { message: 'Hello from the web!' });
            }}>
              Send hello to sidecar
            </Button>
          </div>
        </Wrapper>
      </section>

      {/* second half */}

      <section className="hidden h-screen w-full flex-col items-center justify-center bg-[#d6ebe9] p-9 lg:flex">
        <Image src={robot} alt="Man sitting in wheelchair" />
      </section>
    </section>
  );
}
