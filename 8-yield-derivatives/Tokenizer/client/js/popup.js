document.addEventListener('DOMContentLoaded', function() {
  const supplyBtn = document.querySelector('.supply-button');
  const borrowBtn = document.querySelector('.borrow-button');
  const popup = document.getElementById('popup');
  const closeBtn = document.querySelector('.close');
  const popupTitle = document.getElementById('popupTitle');
  const resourceAddressInput = document.getElementById('resourceAddress');
  const amountAvailableInput = document.getElementById('amountAvailable');
  const totalValueInput = document.getElementById('totalValue');

  function openPopup(type) {
      popup.style.display = 'inline-block';
      popupTitle.textContent = type.charAt(0).toUpperCase() + type.slice(1);
      console.log('Popup opened for type:', type);
      resourceAddressInput.value = '';
      amountAvailableInput.value = '';
      totalValueInput.value = '';
      console.log('OpenPopup ');
  }

  supplyBtn.addEventListener('click', function() {
      openPopup('supply');
  });

  borrowBtn.addEventListener('click', function() {
      openPopup('borrow');
  });

  closeBtn.addEventListener('click', function() {
      popup.style.display = 'none';
  });

  window.addEventListener('click', function(event) {
      if (event.target === popup) {
          popup.style.display = 'none';
      }
  });
});
