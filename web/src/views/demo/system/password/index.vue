<template>
  <PageWrapper title="修改当前用户密码" content="修改成功后会自动退出当前登录！">
    <div class="py-8 bg-white flex flex-col justify-center items-center">
      <BasicForm @register="register" />
      <div class="flex justify-center">
        <a-button @click="resetFields"> 重置 </a-button>
        <a-button class="!ml-4" type="primary" @click="handleSubmit"> 确认 </a-button>
      </div>
    </div>
  </PageWrapper>
</template>
<script lang="ts">
  import { defineComponent } from 'vue';
  import { PageWrapper } from '/@/components/Page';
  import { BasicForm, useForm } from '/@/components/Form';

  import { formSchema } from './pwd.data';
  import {updatePassword} from "@/api/demo/system";
  import {message} from "ant-design-vue";
  import {encrypt} from "@/utils/encrypt";
  import {useRouter} from "vue-router";
  import {PageEnum} from "@/enums/pageEnum";
  import {useUserStoreWithOut} from "@/store/modules/user";

  export default defineComponent({
    name: 'ChangePassword',
    components: { BasicForm, PageWrapper },
    setup() {
      const [register, { validate, resetFields }] = useForm({
        size: 'large',
        baseColProps: { span: 24 },
        labelWidth: 100,
        showActionButtonGroup: false,
        schemas: formSchema,
      });

      async function handleSubmit() {
        try {
          const values = await validate();
          const { oldPassword, newPassword } = values;
          //加密
          const eop = encrypt(oldPassword);
          const enp = encrypt(newPassword);
          //请求
          await updatePassword({
            oldPassword: eop,
            newPassword: enp,
          });
          message.success('修改成功');
          // const { router } = useRouter();
          // router.push(PageEnum.BASE_LOGIN);

          const userStore = useUserStoreWithOut();
          await userStore.logout(true);
        } catch (error) {
          console.log(error)
        }
      }

      return { register, resetFields, handleSubmit };
    },
  });
</script>
