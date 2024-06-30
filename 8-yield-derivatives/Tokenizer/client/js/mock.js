
/**
 * Function for mock table
 */
document.querySelectorAll('.tab-link').forEach(tab => {
    tab.addEventListener('click', function() {
      document.querySelectorAll('.tab-link').forEach(t => t.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
      
      this.classList.add('active');
      document.getElementById(this.getAttribute('data-tab')).classList.add('active');
    });
  });
  
  /**
   * Function for mock table 2
   */
  document.querySelectorAll('.tab-link2').forEach(tab => {
    tab.addEventListener('click', function() {
      document.querySelectorAll('.tab-link2').forEach(t => t.classList.remove('active'));
      document.querySelectorAll('.tab-content2').forEach(c => c.classList.remove('active'));
      
      this.classList.add('active');
      document.getElementById(this.getAttribute('data-tab2')).classList.add('active');
    });
  });
  
  /**
   * Function for mock table
   */
  document.querySelectorAll('.token-button').forEach(button => {
    button.addEventListener('click', function() {
      const token = this.getAttribute('data-token');
      console.log('Token selected:', token);
      // You can add your logic to handle button clicks here
    });
  });
  
  document.querySelectorAll('.yt-button').forEach(button => {
    button.addEventListener('click', function() {
      const token = this.getAttribute('data-token');
      const address = getTokenAddress(token);
      swapYT(address);
    });
  });
  
  document.querySelectorAll('.pt-button').forEach(button => {
    button.addEventListener('click', function() {
      const token = this.getAttribute('data-token');
      const address = getTokenAddress(token);
      swapPT(address);
    });
  });
  
  document.querySelectorAll('.pt-yt-button').forEach(button => {
    button.addEventListener('click', function() {
      const token = this.getAttribute('data-token');
      const address = getTokenAddress(token);
      tradePTYT(address);
    });
  });
  
    
  /**
   * For swapping YT
   * @param {*} resourceAddress 
   */
  function swapYT(resourceAddress) {
    console.log('swapYT called with resource address:', resourceAddress);
    // Your logic for swapping YT
  }
  
  /**
   * For swapping PT
   * @param {*} resourceAddress 
   */
  function swapPT(resourceAddress) {
    console.log('swapPT called with resource address:', resourceAddress);
    // Your logic for swapping PT
  }
  
  /**
   * For trading PT+YT YT
   * @param {*} resourceAddress 
   */
  function tradePTYT(resourceAddress) {
    console.log('tradePT-YT called with resource address:', resourceAddress);
    // Your logic for trading PT-YT
  }
  