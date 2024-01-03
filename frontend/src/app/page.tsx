import Image from "next/image";
import Nav from "../components/Nav";

export default function Home() {
  return (
    <div>
      <Nav />
      <div className="flex flex-col gap-3 p-10 mt-10">
        <div className="flex gap-5 items-center">
          <h1 className="text-2xl font-bold flex-1">New Paste</h1>
          <i>Settings</i>
        </div>
        <textarea className="rounded bg-[#21252b] px-5 py-3 h-[50vh]"></textarea>
        <button className="bg-green-500 rounded-lg ms-auto text-black font-bold border-2 border-black py-3 px-5">
          Create New Paste
        </button>
      </div>
    </div>
  );
}
