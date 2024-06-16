import Image from "next/image";
import styles from "./navbar.module.css";
const Navbar = () => {
  return (
    <div className={styles.container}>
      <div className={styles.brand}>
        <Image src={"/toyboat_brand.png"} alt="" width="50" height="50" />
      </div>
      <div className={styles.searchBar}>
        <input type="text" placeholder="Search" />
      </div>
      <div className={styles.userInfo}>
        <div className={styles.name}>User PH</div>
        <div className={styles.wallet}>Wallet PH</div>
      </div>
    </div>
  )
}

export default Navbar;
